name: Build and Push Processor

on:
  push:
    branches:
      - dev
      - prod
env:
  CARGO_TERM_COLOR: always

jobs:
  # deploy on pushes to master branch
  # assumes aws credentials (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY) are set in github secrets
  push_dev:
    if: github.ref == 'refs/heads/dev'
    runs-on: ubuntu-latest
    env:
      TFE_TOKEN: ${{ secrets.TFE_TOKEN }}
      SLS_DOCKER_ARGS: -v  /home/runner/work/example-service/example-service/example-app:/example-app
    steps:

      - name: Checkout
        uses: actions/checkout@v2
        
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v1
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: ap-southeast-1

      - name: Deploy Terraform Config 
        run: |
          chmod +x ./scripts/terraform-enterprise-push.sh
          ./scripts/terraform-enterprise-push.sh example-dynamodb-lambda/terraform HocVienCongGiao/dev-sg-lambda-processors-dynamodb-example-service
