resource "aws_db_subnet_group" "main" {
  name       = "${var.environment}-db-subnet-group"
  subnet_ids = var.private_subnet_ids

  tags = {
    Name = "${var.environment}-db-subnet-group"
  }
}

resource "aws_security_group" "rds" {
  name   = "${var.environment}-rds-sg"
  vpc_id = var.vpc_id

  ingress {
    from_port   = 5432
    to_port     = 5432
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "${var.environment}-rds-sg"
  }
}

resource "aws_db_instance" "main" {
  identifier            = "${var.environment}-scavenger-db"
  engine                = "postgres"
  engine_version        = "15.3"
  instance_class        = var.instance_class
  allocated_storage     = var.allocated_storage
  storage_type          = "gp3"
  db_name               = var.db_name
  username              = var.db_username
  password              = var.db_password
  db_subnet_group_name  = aws_db_subnet_group.main.name
  vpc_security_group_ids = [aws_security_group.rds.id]
  multi_az              = var.multi_az
  backup_retention_period = var.backup_retention
  backup_window         = "03:00-04:00"
  maintenance_window    = "sun:04:00-sun:05:00"
  skip_final_snapshot   = false
  final_snapshot_identifier = "${var.environment}-scavenger-db-final-snapshot-${formatdate("YYYY-MM-DD-hhmm", timestamp())}"
  enabled_cloudwatch_logs_exports = ["postgresql"]
  storage_encrypted     = true
  deletion_protection   = var.environment == "prod" ? true : false

  tags = {
    Name = "${var.environment}-scavenger-db"
  }
}

resource "aws_db_instance" "read_replica" {
  count              = var.multi_az ? 1 : 0
  identifier         = "${var.environment}-scavenger-db-replica"
  replicate_source_db = aws_db_instance.main.identifier
  instance_class     = var.instance_class
  skip_final_snapshot = true

  tags = {
    Name = "${var.environment}-scavenger-db-replica"
  }
}
