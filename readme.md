creer son autorité de certification (rootCA_key.key et rootCA_cert.cer)


sign 
openssl ca -batch -config ca.conf -notext -in intermediate1.csr -out intermediate1_cert.crt