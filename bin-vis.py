import sys

if len(sys.argv) != 4:
	print('Usage: op num1 num2')
	quit()

op = sys.argv[1]
a = int(sys.argv[2])
b = int(sys.argv[3])

# binary strings of input
a_bin = '{0:b}'.format(a).zfill(8)

b_bin = '{0:b}'.format(b).zfill(8)

print(f'{a}\t{a_bin:>15}')
print(f'{b}\t{b_bin:>15}')

if op == 'add':
	res = a + b if a + b < 256 else a + b - 256

elif op == 'sub':
	res = a - b if a - b >= 0 else a - b + 256
	
else:
	print(f'op "{op}" is not legal')
	quit()


res_bin = '{0:b}'.format(res).zfill(8)

print(f'{res}\t{res_bin:>15}')

