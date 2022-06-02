npm config set @deland-labs:registry https://www.myget.org/F/ic-feed/npm/
npm install -g yarn tsconfig-paths
npm install

npx icdev init-identity
npx ts-node -r tsconfig-paths/register scripts/index.ts update-local-config
pre-commit install
pipenv install
