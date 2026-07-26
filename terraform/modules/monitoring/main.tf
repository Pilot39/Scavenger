resource "aws_cloudwatch_metric_alarm" "alb_target_response_time" {
  alarm_name          = "${var.environment}-alb-response-time"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "TargetResponseTime"
  namespace           = "AWS/ApplicationELB"
  period              = "60"
  statistic           = "Average"
  threshold           = "1"
  alarm_description   = "Alert when ALB response time exceeds 1 second"

  dimensions = {
    LoadBalancer = var.alb_arn
  }
}

resource "aws_cloudwatch_metric_alarm" "ecs_cpu_utilization" {
  alarm_name          = "${var.environment}-ecs-cpu-utilization"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ECS"
  period              = "300"
  statistic           = "Average"
  threshold           = "80"
  alarm_description   = "Alert when ECS CPU exceeds 80%"

  dimensions = {
    ClusterName = var.ecs_cluster
  }
}

resource "aws_cloudwatch_metric_alarm" "rds_cpu_utilization" {
  alarm_name          = "${var.environment}-rds-cpu-utilization"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "2"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/RDS"
  period              = "300"
  statistic           = "Average"
  threshold           = "80"
  alarm_description   = "Alert when RDS CPU exceeds 80%"

  dimensions = {
    DBInstanceIdentifier = var.rds_id
  }
}

resource "aws_cloudwatch_metric_alarm" "rds_storage" {
  alarm_name          = "${var.environment}-rds-storage"
  comparison_operator = "GreaterThanThreshold"
  evaluation_periods  = "1"
  metric_name         = "FreeStorageSpace"
  namespace           = "AWS/RDS"
  period              = "300"
  statistic           = "Average"
  threshold           = "2147483648" # 2GB
  alarm_description   = "Alert when RDS free storage < 2GB"

  dimensions = {
    DBInstanceIdentifier = var.rds_id
  }
}
