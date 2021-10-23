macro IN_STO(location) = {
	IN
	COMPLEX_STO!(location)
}

macro COMPLEX_STO(location) = {
	STO location
}

# Code to compute a divided by b
	IN_STO!(a)
	IN_STO!(b)
# Keep subtracting a from b until you go negative
# Keep a count of how many times you do it
start	LDA	count
	ADD	one
	STO	count
	LDA	a
	SUB	b
	STO	a
	BRP	start
done	LDA	count
# Subtract one as we went one too far
	SUB	one
	OUT
	HLT
a	DAT	000
b	DAT	000
count	DAT	000
one	DAT	001