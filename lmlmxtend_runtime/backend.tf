terraform {
  backend "s3" {
    bucket       = "your-org-terraform-state-prod"
    key          = "lmlmxtend-runtime/prod/terraform.tfstate"
    region       = "us-east-1"
    encrypt      = true
    use_lockfile = true
  }
}
