NETWORK=devnet


KEYPAIR_AUTH=./.keypairs/${NETWORK}/ido5zRdfphtyeov8grJqBszfWquvTCAvRzGycSfPXuX.json
KEYPAIR_PROGRAM=./.keypairs/${NETWORK}/ido5zRdfphtyeov8grJqBszfWquvTCAvRzGycSfPXuX.json

env:
	solana config set --url ${NETWORK}
	solana config set --keypair  ${KEYPAIR_AUTH}
	cp ${KEYPAIR_AUTH} ~/.config/solana/id.json
	mkdir -p ./target/deploy && cp ${KEYPAIR_PROGRAM} ./target/deploy/ido_protocol-keypair.json || true # this can safely fail


airdrop:
	solana --url ${NETWORK} \
	--keypair ${KEYPAIR_AUTH} \
	airdrop 1

build:
	anchor build

deploy: build
	solana program deploy ./target/deploy/ido_protocol.so \
	--url ${NETWORK} \
	--program-id ${KEYPAIR_PROGRAM} \
	--keypair ${KEYPAIR_AUTH}
	say -v Samantha protocol deployment completed || true

deploy-resume: build
	solana program deploy ./target/deploy/ido_protocol.so \
	--url ${NETWORK} \
	--program-id ${KEYPAIR_PROGRAM} \
	--keypair ${KEYPAIR_AUTH} \
	--buffer $(buffer)
	say -v Samantha protocol deployment completed || true

test:
	#make airdrop || true
	anchor test

test-skip:
	anchor test --skip-build --skip-deploy

create-pool:
	ts-mocha -t 1000000 create-pool-example.ts

