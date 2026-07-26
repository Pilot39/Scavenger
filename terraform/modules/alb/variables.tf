variable "environment" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "public_subnet_ids" {
  type = list(string)
}

variable "target_group_arn" {
  type = string
}

variable "certificate_arn" {
  type = string
}
