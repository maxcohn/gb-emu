
rest = None
with open('src/cpu.rs') as f:
	for line in f:
		if line.strip().startswith('match cur_op'):
			break
	
	rest = f.readlines()

implemented_codes = []
for line in rest:
	if not line.strip().startswith('0x'):
		continue
	code = int(line.strip()[:4], base=16)
	implemented_codes.append(code)

implemented_codes.sort()

for i in range(256):
	if i  in implemented_codes:
		print(hex(i))
