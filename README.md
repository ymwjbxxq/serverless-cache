# Serverless Cache

This project is an idea that I had to solve some performance problems while exposing data across regions.
I did not want to use a costly cluster in each region or point everybody to one region, so I came up with the Serverless Cache.

Necessary: this application uses various AWS services, and there are costs associated with these services after the Free Tier usage - please see the [AWS Pricing page](https://aws.amazon.com/pricing/) for details. You are responsible for any AWS costs incurred. No warranty is implied in this example.

## How it works

Please read this article []()

## Requirements

* [Create an AWS account](https://portal.aws.amazon.com/gp/aws/developer/registration/index.html). If you do not already have one, log in. The IAM user that you use must have sufficient permissions to make necessary AWS service calls and manage AWS resources.
* [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) installed and configured
* [Git Installed](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* [AWS Serverless Application Model](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) (AWS SAM) installed
* [Rust](https://www.rust-lang.org/) 1.56.0 or higher
* [cargo-zigbuild](https://github.com/messense/cargo-zigbuild) and [Zig](https://ziglang.org/) for cross-compilation

## Deployment Instructions

1. Create a new directory, navigate to that directory in a terminal and clone the GitHub repository:
    ``` 
    git clone https://github.com/ymwjbxxq/serverless-cache
    ```
2. Change the directory to the pattern directory:
    ```
    cd serverless-cache
    ```
3. Install dependencies and build:
    ```
    make build
    ```
4. Deploy the API on the PrimaryRegion region.
    ```
    sam deploy --guided --stack-name sam-dynamodb --template-file dynamodb.yml
    ```
5. Deploy the cache service API on the PrimaryRegion region.
    ```

    sam deploy --guided --region {PrimaryRegion} --stack-name sam-api-primary --template-file api.yml --parameter-overrides TableName={DYNAMODB_TABLE_NAME}  DistributionDomainName={DistributionDomainName}
    ```

6. Deploy the cache service API on the SecondaryRegion region.
    ```
    sam deploy --guided --region {SecondaryRegion} --stack-name sam-api-secondary --template-file api.yml --parameter-overrides TableName={DYNAMODB_TABLE_NAME}
    ```

7. Deploy CloudFront with a failover policy.
    ```
    sam deploy --guided --region {PrimaryRegion} --stack-name sam-cf --template-file cloudfront.yml --parameter-overrides ApiPrimaryEndpoint={HTTP_API_ID}.execute-api.{PrimaryRegion}.amazonaws.com ApiSecondaryEndpoint={HTTP_API_ID}.execute-api.{SecondaryRegion}.amazonaws.com
    ```
8. Because this PoC does not use a custom domain at the CloudFront level, the Lambda template has created the set-cache Lambda with a TODO reference to fill.
You must set DISTRIBUTION_CUSTOM_DOMAIN_NAME with CloudFront domain name in the set-cache lambda environment variable. 

5. Deploy the origin API. Because this PoC the route53 custom domain is not created and so you must pass the domain name of one of the cache API  
    ```
    sam deploy --guided --region {PrimaryRegion} --stack-name sam-api-origin --template-file origin-api.yml --parameter-overrides CacheEndpoint={HTTP_API_ID}.execute-api.{PrimaryRegion}.amazonaws.com


## Testing with already warming up cache

Once the application is deployed, retrieve the CloudFront value from CloudFormation Outputs. Then, either browse to the endpoint in a web browser or call the endpoint from Postman.

Example POST Request:
curl -d '{ "key": "myKey", "data": "new data", "cdnTTL": 120,   "clientTTL": 10}' -H "Content-Type: application/json" -X POST https://{HTTP_API_ID}.execute-api.{PrimaryRegion}.amazonaws.com/set-cache

Response: it will be a 200

Now we check if the key is cached.

https://{DistributionDomainName}/cache?key=myKey

If you check the headers back from the call, you will see:

If you try again:
X-Cache: Hit from CloudFront

Response:
```
"new data"
```

## Testing without warming up cache

Once the application is deployed, retrieve the CloudFront value from CloudFormation Outputs. Then, either browse to the endpoint in a web browser or call the endpoint from Postman.

https://{DistributionDomainName}/cache?key=myKey&origin_url=https://{OriginDomain}/origin?key=myKey

If you check the headers back from the call, you will see:

X-Cache: Miss from cloudfront

If you try again:
X-Cache: Hit from CloudFront

Response:
```
some JSON
```

## Documentation
- [Working with HTTP APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api.html)
- [Working with AWS Lambda proxy integrations for HTTP APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html)
- [AWS Lambda - the Basics](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/aws-lambdathe-basics.html)
- [Lambda Function Handler](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-handler.html)
- [Function Event Object - Overview](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-event-object.html)
- [Function Event Object - HTTP API v2 Event](https://github.com/awsdocs/aws-lambda-developer-guide/blob/master/sample-apps/nodejs-apig/event-v2.json)
- [Function Context Object - Overview](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-context-object.html)
- [Function Context Object in Node.js - Properties](https://docs.aws.amazon.com/lambda/latest/dg/nodejs-context.html)
- [Function Environment Variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html)
- [Amazon CloudFront](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html)
- [Add a custom domain managed by Amazon Route 53](https://docs.aws.amazon.com/amplify/latest/userguide/to-add-a-custom-domain-managed-by-amazon-route-53.html)

## Cleanup
 
1. Delete the stack
    ```bash
    aws cloudformation delete-stack --stack-name STACK_NAME
    ```
2. Confirm the stack has been deleted
    ```bash
    aws cloudformation list-stacks --query "StackSummaries[?contains(StackName,'STACK_NAME')].StackStatus"
    ```

----

