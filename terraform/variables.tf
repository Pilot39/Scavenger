variable "aws_region" {
  type        = string
  default     = "us-east-1"
  description = "AWS region"
}

variable "environment" {
  type        = string
  description = "Environment name (dev, staging, prod)"
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be dev, staging, or prod."
  }
}

variable "vpc_cidr" {
  type        = string
  default     = "10.0.0.0/16"
  description = "VPC CIDR block"
}

variable "az_count" {
  type        = number
  default     = 3
  description = "Number of availability zones"
}

variable "db_name" {
  type        = string
  default     = "scavenger"
  description = "Database name"
}

variable "db_username" {
  type        = string
  description = "Database master username"
  sensitive   = true
}

variable "db_password" {
  type        = string
  description = "Database master password"
  sensitive   = true
}

variable "db_instance_class" {
  type        = string
  default     = "db.t3.micro"
  description = "RDS instance class"
}

variable "db_allocated_storage" {
  type        = number
  default     = 20
  description = "Allocated storage in GB"
}

variable "db_backup_retention" {
  type        = number
  default     = 30
  description = "Backup retention period in days"
}

variable "db_multi_az" {
  type        = bool
  default     = true
  description = "Enable Multi-AZ deployment"
}

variable "container_image" {
  type        = string
  description = "Docker image URI"
}

variable "container_port" {
  type        = number
  default     = 8080
  description = "Container port"
}

variable "ecs_desired_count" {
  type        = number
  default     = 3
  description = "Desired number of ECS tasks"
}

variable "ecs_min_capacity" {
  type        = number
  default     = 3
  description = "Minimum ECS capacity"
}

variable "ecs_max_capacity" {
  type        = number
  default     = 10
  description = "Maximum ECS capacity"
}

variable "certificate_arn" {
  type        = string
  description = "ACM certificate ARN for HTTPS"
}

variable "ci_role_arn" {
  type        = string
  description = "IAM role ARN for CI/CD image push"
}

variable "ecs_task_role_arn" {
  type        = string
  description = "IAM role ARN for ECS task image pull"
}

variable "alerts_sns_arn" {
  type        = string
  description = "SNS topic ARN for infrastructure alerts"
}

variable "ecs_service_name" {
  type        = string
  description = "ECS service name"
  default     = "scavenger-backend"
}

variable "spot_asg_arn" {
  type        = string
  description = "ARN of the Spot Auto Scaling Group for ECS"
}

variable "monthly_budget_usd" {
  type        = string
  default     = "200"
  description = "Monthly AWS budget limit in USD"
}

variable "budget_alert_emails" {
  type        = list(string)
  description = "Email addresses for budget alerts"
  default     = []
}

variable "scheduler_role_arn" {
  type        = string
  description = "IAM role ARN for EventBridge scheduler (RDS auto-stop)"
}
