from matplotlib import pyplot as plt

f = open("input")

lines: list[str] = f.read().split('\n')

x: list[int] = []
y: list[int] = []

for line in lines:
	a, b = line.split(',')
	x.append(int(a))
	y.append(int(b))

a, b = lines[0].split(',')
x.append(int(a))
y.append(int(b))

plt.plot(x, y)
plt.show()