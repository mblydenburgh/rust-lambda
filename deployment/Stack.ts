import cdk = require('@aws-cdk/core');
import { Code, Function, Runtime } from "@aws-cdk/aws-lambda"
import { Effect, ManagedPolicy, PolicyStatement, Role, ServicePrincipal } from '@aws-cdk/aws-iam';
import { AttributeType, BillingMode, Table } from "@aws-cdk/aws-dynamodb"
import { CorsHttpMethod, HttpApi, HttpMethod } from "@aws-cdk/aws-apigatewayv2"
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations"
import { Aws } from '@aws-cdk/core';
export class CdkStack extends cdk.Stack {
    constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const appName = "rust-lambda-test"

        const dynamoTable = new Table(this, `DynamoTable`, {
            tableName: `${appName}-table`,
            partitionKey: { name: "uuid", type: AttributeType.STRING },
            sortKey: { name: "last_name", type: AttributeType.STRING },
            billingMode: BillingMode.PAY_PER_REQUEST
        })

        const lambdaRole = new Role(this, `LambdaRole`, {
            roleName: "LambdaExecutionRole",
            assumedBy: new ServicePrincipal("lambda.amazonaws.com"),
            managedPolicies: [
                ManagedPolicy.fromAwsManagedPolicyName("service-role/AWSLambdaVPCAccessExecutionRole")
            ]
        })

        dynamoTable.grantReadWriteData(lambdaRole)
        lambdaRole.addToPolicy(new PolicyStatement({
            effect: Effect.ALLOW,
            actions: ["dynamodb:query"],
            resources: [
                `arn:aws:dynamodb:${Aws.REGION}:${Aws.ACCOUNT_ID}:table/${dynamoTable.tableName}/index/*`
            ]
        }))

        const lambdaFunction = new Function(this, `LambdaFunction`, {
            functionName: `${appName}-lambda`,
            runtime: Runtime.FROM_IMAGE,
            role: lambdaRole,
            code: Code.fromAsset("../bin/main.zip"),
            handler: "main"
        })
        lambdaFunction.grantInvoke(new ServicePrincipal("apigateway.amazonaws.com"))

        const lambdaIntegration = new HttpLambdaIntegration("HttpLambdaIntegration", lambdaFunction)

        const api = new HttpApi(this, `RestAPIGateway`, {
            apiName: "rust-api",
            corsPreflight: {
                allowHeaders: ['Authorization', 'Access-Control-Allow-Origin','Access-Control-Allow-Headers','Content-Type',"X-Api-Key","X-Amz-Security-Token"],
                allowMethods: [
                    CorsHttpMethod.ANY
                ],
                allowOrigins: ['*'],
            },
        })

        api.addRoutes({
            path: "/users",
            methods: [HttpMethod.ANY],
            integration: lambdaIntegration
        })

        api.addRoutes({
            path: "/users/{proxy+}",
            methods: [HttpMethod.ANY],
            integration: lambdaIntegration
        })
    }
}