# Terraform Infrastructure as Code Documentation

## Overview
This directory contains Terraform configurations for deploying the Scavenger platform on AWS.

## Structure
```
terraform/
├── main.tf                 # Root module configuration
├── variables.tf            # Input variables
├── environments/           # Environment-specific configurations
│   ├── prod.tfvars
│   ├── staging.tfvars
│   └── dev.tfvars
└── modules/               # Reusable modules
    ├── vpc/              # VPC and networking
    ├── rds/              # Database
    ├── ecs/              # Container orchestration
    ├── alb/              # Load balancing
    └── monitoring/       # CloudWatch monitoring
```

## Prerequisites
- Terraform >= 1.0
- AWS CLI configured with credentials
- S3 bucket for state management
- DynamoDB table for state locking

## State Management
State is stored in S3 with DynamoDB locking:
```bash
# Create S3 bucket
aws s3 mb s3://scavenger-terraform-state --region us-east-1

# Create DynamoDB table
aws dynamodb create-table \
  --table-name terraform-locks \
  --attribute-definitions AttributeName=LockID,AttributeType=S \
  --key-schema AttributeName=LockID,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5
```

## Deployment

### Initialize
```bash
terraform init
```

### Plan
```bash
terraform plan -var-file=environments/prod.tfvars
```

### Apply
```bash
terraform apply -var-file=environments/prod.tfvars
```

### Destroy
```bash
terraform destroy -var-file=environments/prod.tfvars
```

## Pre-commit Hooks
Install pre-commit hooks for validation:
```bash
pre-commit install
```

Hooks validate:
- Terraform formatting
- Terraform validation
- Security scanning with tfsec
- Cost estimation with infracost
