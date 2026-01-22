-- Active: 1768790788267@@127.0.0.1@3306
INSERT INTO asset (name, symbol, kind)
VALUES ('Polkadot', 'DOT', 'cryptocoin');

INSERT INTO asset (name, symbol, kind)
VALUES ('Bitcoin', 'BTC', 'cryptocoin');

INSERT INTO crypto_exchanger (name,base_url)
VALUES ('coinmarketcap', 'https://api.coinmarketcap' )

INSERT INTO asset_crypto_exchanger (asset_id,crypto_exchanger_id,priority,enabled)
VALUES (1,1,1,true)