variable "environment" {
  type        = string
  description = "Environment name"
}

variable "ci_role_arn" {
  type        = string
  description = "IAM role ARN used by CI/CD to push images"
}

variable "ecs_task_role_arn" {
  type        = string
  description = "IAM role ARN used by ECS tasks to pull images"
}

variable "alerts_sns_arn" {
  type        = string
  description = "SNS topic ARN for critical scan finding alerts"
}
