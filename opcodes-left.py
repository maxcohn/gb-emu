import re

rest = None
with open('src/cpu.rs') as f:
	for line in f:
		if line.strip().startswith('match cur_op'):
			break
	
	rest = ''.join(f.readlines())

RE_HEX = re.compile(r'0x[0-9A-F][0-9A-F]')

implemented_codes = []
for m in RE_HEX.finditer(rest):
	implemented_codes.append(int(m.group(0), base=16))
'''
implemented_codes = []
for line in rest:
	if not line.strip().startswith('0x'):
		continue
	if line.strip()[4:6] == '..':
		code_start = int(line.strip()[:4], base=16)
		code_end = int(line.strip()[6:10], base=16)
		for i in range(code_start, code_end + 1):
			implemented_codes.append(i)
	else:
		code = int(line.strip()[:4], base=16)
		implemented_codes.append(code)

implemented_codes = list(dict.fromkeys(implemented_codes))
implemented_codes.sort()


'''
count = 0
for i in range(256):
	if i not in implemented_codes:
		print(hex(i))
		count += 1

print(f'Number of codes left: {count}')
