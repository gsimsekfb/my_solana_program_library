
## Example of creating and minting new fungible token using Solana spl-token cli 

11-Dec-23
user1: 7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc

### 1. Create new fungible token
```
spl-token create-token
Creating token 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo 
under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo // new Token
Decimals:  9

Signature: 4HeTaaNRkd6E9dDWfhPpL2rXUpsfuME5kjhcLFpgiy3wDJGDSAGqxDYR7MGW56MdH2cARvFoubUNBXTQz2zDSeyY
``` 
// Newly created Token's addr:
8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
    https://explorer.solana.com/address/8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo?cluster=devnet
// Mint Authority is user1
7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc


### 2. Create Associated Token Account of user1 for token 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
```
spl-token create-account 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
Creating account CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr9

Signature: 2ECvYmSG9yEdKStKas4N6ZdpARW9Mx1yURJaspbwnXkGSeMMqaheWAsSHpEnyMBb2VynFXyj2GxKQPFPaRWc163C
``` 
// user1's Associated Token Account CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr 
https://explorer.solana.com/address/CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr9?cluster=devnet
Address	        CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr
Mint            8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
Owner           7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc
Token balance	0


### 3. Mint 500 token 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
solana gsimsek$ spl-token mint 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo 500
Minting 500 tokens
  Token: 8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
  Recipient: CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr9

Signature: ds7JDtBn1pTd8h8o7FJuYzp1mRR7QVUM262HuCSGrK2EqAToHCoZmgWDSN8RwLpykHb4SDToCW94onicyxpyZNy

//
https://explorer.solana.com/address/CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr9?cluster=devnet
Address	        CZgEVtGH3og53bMDjpVnkcHE4rZsTh5Th3WKChqWKpr
Mint            8RPk9U6x3Nvc5M3yyvfMzT3iofztgT4Rry7r4Cgi8woo
Owner           7GDXzkmtqNG2BZmesUyv2qrbRoovv71TApd1bWSsZAuc
Token balance	0