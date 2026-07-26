variable "environment" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "private_subnet_ids" {
  type = list(string)
}

variable "db_name" {
  type = string
}

variable "db_username" {
  type      = string
  sensitive = true
}

variable "db_password" {
  type      = string
  sensitive = true
}

variable "instance_class" {
  type = string
}

variable "allocated_storage" {
  type = number
}

variable "backup_retention" {
  type = number
}

variable "multi_az" {
  type = bool
}
