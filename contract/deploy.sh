#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

should_reset=0
for arg in "$@"
do
  if [ "$arg" == "-r" ] ; then
    should_reset=1
    break
  fi
done
 
if [ $should_reset == 1 ] ; then
  echo ">> Remove neardev dir"

  rm -rf neardev || true
fi

echo ">> Deploying contract"

if [ $should_reset == 1 ] ; then
  near dev-deploy --wasmFile res/sweat_burn.wasm --initFunction "new" --initArgs '{"token_account_id": "vfinal.token.sweat.testnet", "authorized_accounts": []}'
else
  near dev-deploy --wasmFile res/sweat_burn.wasm
fi