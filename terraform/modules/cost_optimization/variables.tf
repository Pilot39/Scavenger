variable "environment" {
  type = string
}

variable "aws_region" {
  type    = string
  default = "us-east-1"
}

variable "ecs_cluster_name" {
  type        = string
  description = "ECS cluster name"
}

variable "ecs_service_name" {
  type        = string
  description = "ECS service name to auto-scale"
}

variable "ecs_min_capacity" {
  type    = number
  default = 1
}

variable "ecs_max_capacity" {
  type    = number
  default = 10
}

variable "spot_asg_arn" {
  type        = string
  description = "ARN of the Spot Auto Scaling Group"
}

variable "monthly_budget_usd" {
  type        = string
  default     = "200"
  description = "Monthly budget limit in USD"
}

variable "budget_alert_emails" {
  type        = list(string)
  description = "Email addresses for budget alerts"
}

variable "enable_org_tagging_policy" {
  type        = bool
  default     = false
  description = "Enable AWS Organizations tag policy (requires Organizations)"
}

variable "rds_instance_id" {
  type        = string
  description = "RDS instance ID for auto-stop/start in non-prod"
}

variable "scheduler_role_arn" {
  type        = string
  description = "IAM role ARN for EventBridge to invoke SSM automation"
}
