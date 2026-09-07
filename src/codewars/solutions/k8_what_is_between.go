package kata

func Between(a, b int) (c []int) {
	for a <= b {
		c = append(c, a)
		a++
	}
	return
}
