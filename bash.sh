aws s3api create-bucket \
  --bucket your-org-terraform-state-prod \
  --region us-east-1

aws s3api put-bucket-versioning \
  --bucket your-org-terraform-state-prod \
  --versioning-configuration Status=Enabled

aws s3api put-bucket-encryption \
  --bucket your-org-terraform-state-prod \
  --server-side-encryption-configuration '{
    "Rules": [{"ApplyServerSideEncryptionByDefault": {"SSEAlgorithm": "AES256"}}]
  }'

aws s3api put-public-access-block \
  --bucket your-org-terraform-state-prod \
  --public-access-block-configuration \
    BlockPublicAcls=true,IgnorePublicAcls=true,BlockPublicPolicy=true,RestrictPublicBuckets=true



bash scripts/new-extension.sh --name ecg-processor --category signal --capability signal.ecg.analyze
cargo test -p lmlmxtend-ext-ecg-processor -- --nocapture
terraform init
terraform validate
terraform plan
