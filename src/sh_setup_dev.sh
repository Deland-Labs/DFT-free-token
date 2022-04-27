npm install
npm i typescript -g
npm i ts-node -g
npm i tsconfig-paths -g
ts-node -r tsconfig-paths/register scripts/createIdentities.ts
ts-node -r tsconfig-paths/register scripts/updateLocalConfigs.ts
pre-commit install
pipenv install
