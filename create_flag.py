import struct

flag = b"m1x_Vm_R3al_cPus"

x0 = 0xd5702daa
x2 = 0xf2d4ad0c
x4 = 0xb95834c9
x6 = 0x17ae510d
x8 = 0xb8dfefa0
x10 = 0xb7bd13d1
x12 = 0x87e8f006

def apply_xor(offset, value, xor):
    v = value[offset:offset + 4]
    vint = struct.unpack('<I', v)[0]
    vint ^= xor
    v = struct.pack('<I', vint)
    return value[:offset] + v + value[offset + 4:]

flag = apply_xor(12, flag, x12)
flag = apply_xor(10, flag, x10)
flag = apply_xor(8, flag, x8)
flag = apply_xor(6, flag, x6)
flag = apply_xor(4, flag, x4)
flag = apply_xor(2, flag, x2)
flag = apply_xor(0, flag, x0)

# print flag as hex
for i in range(0, 16, 4):
    print(flag[i:i + 4][::-1].hex())
