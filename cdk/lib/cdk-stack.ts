import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Code, Function, Runtime } from "@aws-cdk/aws-lambda"
import { Effect, ManagedPolicy, PolicyStatement, Role, ServicePrincipal } from '@aws-cdk/aws-iam';
import { AttributeType, BillingMode, Table } from "@aws-cdk/aws-dynamodb"
import { CorsHttpMethod, HttpApi, HttpMethod } from "@aws-cdk/aws-apigatewayv2"
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations"
import { Aws } from '@aws-cdk/core';

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const appName = "rust-lambda"

    //@ts-ignore
    const dynamoTable = new Table(this, `DynamoTable`, {
      tableName: `${appName}-table`,
      partitionKey: { name: "userId", type: AttributeType.STRING },
      sortKey: { name: "modelTypeAndId", type: AttributeType.STRING },
      billingMode: BillingMode.PAY_PER_REQUEST
    })

    //@ts-ignore
    const lambdaRole = new Role(this, `LambdaRole`, {
      roleName: `${appName}-role`,
      assumedBy: new ServicePrincipal("lambda.amazonaws.com"),
      managedPolicies: [
        ManagedPolicy.fromAwsManagedPolicyName("service-role/AWSLambdaVPCAccessExecutionRole")
      ]
    })

    dynamoTable.grantReadWriteData(lambdaRole)
    lambdaRole.addToPolicy(new PolicyStatement({
      effect: Effect.ALLOW,
      actions: ["dynamodb:Query", "dynamodb:Scan", "dynamodb:GetItem", "dynamodb:PutItem", "dynamodb:DeleteItem"],
      resources: [
        `arn:aws:dynamodb:${Aws.REGION}:${Aws.ACCOUNT_ID}:table/${dynamoTable.tableName}/index/*`
      ]
    }))

    //@ts-ignore
    const lambdaFunction = new Function(this, `LambdaFunction`, {
      functionName: `${appName}-lambda`,
      runtime: Runtime.PROVIDED_AL2,
      role: lambdaRole,
      code: Code.fromAsset("../target/x86_64-unknown-linux-musl/release/lambda.zip"),
      handler: "main",
      environment: {
        RUST_BACKTRACE: '1'
      }
    })
    lambdaFunction.grantInvoke(new ServicePrincipal("apigateway.amazonaws.com"))

    const lambdaIntegration = new HttpLambdaIntegration("HttpLambdaIntegration", lambdaFunction)

    //@ts-ignore
    const api = new HttpApi(this, `RestAPIGateway`, {
      apiName: "rust-lambda-api",
      corsPreflight: {
        allowHeaders: ['Authorization', 'Access-Control-Allow-Origin', 'Access-Control-Allow-Headers', 'Content-Type'],
        allowMethods: [
          CorsHttpMethod.ANY
        ],
        allowOrigins: ['*'],
      },
    })

    api.addRoutes({
      path: "/users",
      methods: [HttpMethod.GET, HttpMethod.POST],
      integration: lambdaIntegration
    })

    api.addRoutes({
      path: "/users/{proxy+}",
      methods: [HttpMethod.GET, HttpMethod.POST],
      integration: lambdaIntegration
    })
  }
}
