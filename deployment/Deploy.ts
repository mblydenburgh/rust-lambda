import cdk = require("@aws-cdk/core")
import { CdkStack } from "./Stack";

const app = new cdk.App();
const accountNumber = "415023725722"

const stack = new CdkStack(app, `rust-lambda-stack`, {
    env: {
        region: "us-east-1",
        account: accountNumber
    }
});