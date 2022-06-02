# get first argument as version
VERSION=$1
# error if no version
if [ -z "$VERSION" ]; then
  echo "Error: No version specified"
  exit 1
fi
npx icdev pack \
    --package-scope deland-labs \
    --canister-env-name EX3_CANISTER_ENV \
    --production-canister-env prod \
    --publish \
    --package-version $VERSION
