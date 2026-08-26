locals {
  repos = ["backend", "indexer", "frontend"]
}

resource "aws_ecr_repository" "this" {
  for_each = toset(local.repos)

  name                 = "scavenger/${each.key}"
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  encryption_configuration {
    encryption_type = "KMS"
    kms_key         = aws_kms_key.ecr.arn
  }

  tags = {
    Name = "scavenger-${each.key}"
  }
}

# KMS key for ECR encryption
resource "aws_kms_key" "ecr" {
  description             = "KMS key for ECR repositories"
  deletion_window_in_days = 7
  enable_key_rotation     = true
}

resource "aws_kms_alias" "ecr" {
  name          = "alias/scavenger-ecr-${var.environment}"
  target_key_id = aws_kms_key.ecr.key_id
}

# Lifecycle policy: keep last 10 tagged + remove untagged after 1 day
resource "aws_ecr_lifecycle_policy" "this" {
  for_each   = aws_ecr_repository.this
  repository = each.value.name

  policy = jsonencode({
    rules = [
      {
        rulePriority = 1
        description  = "Remove untagged images after 1 day"
        selection = {
          tagStatus   = "untagged"
          countType   = "sinceImagePushed"
          countUnit   = "days"
          countNumber = 1
        }
        action = { type = "expire" }
      },
      {
        rulePriority = 2
        description  = "Keep last 10 tagged images"
        selection = {
          tagStatus     = "tagged"
          tagPrefixList = ["v", "sha-"]
          countType     = "imageCountMoreThan"
          countNumber   = 10
        }
        action = { type = "expire" }
      }
    ]
  })
}

# Repository policy: restrict push to CI role, allow pull from ECS task role
resource "aws_ecr_repository_policy" "this" {
  for_each   = aws_ecr_repository.this
  repository = each.value.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "AllowPushFromCI"
        Effect = "Allow"
        Principal = {
          AWS = var.ci_role_arn
        }
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:CompleteLayerUpload",
          "ecr:InitiateLayerUpload",
          "ecr:PutImage",
          "ecr:UploadLayerPart"
        ]
      },
      {
        Sid    = "AllowPullFromECS"
        Effect = "Allow"
        Principal = {
          AWS = var.ecs_task_role_arn
        }
        Action = [
          "ecr:BatchGetImage",
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchCheckLayerAvailability"
        ]
      }
    ]
  })
}

# Enable ECR image scanning findings notifications via EventBridge
resource "aws_cloudwatch_event_rule" "ecr_scan" {
  name        = "scavenger-ecr-scan-findings-${var.environment}"
  description = "Trigger on ECR image scan findings"

  event_pattern = jsonencode({
    source      = ["aws.ecr"]
    detail-type = ["ECR Image Scan"]
    detail = {
      "finding-severity-counts" = {
        CRITICAL = [{ numeric = [">", 0] }]
      }
    }
  })
}

resource "aws_cloudwatch_event_target" "ecr_scan_sns" {
  rule      = aws_cloudwatch_event_rule.ecr_scan.name
  target_id = "SendToSNS"
  arn       = var.alerts_sns_arn
}
