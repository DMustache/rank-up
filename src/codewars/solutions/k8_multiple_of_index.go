package kata

func MultipleOfIndex(ints []int) (result []int) {
	for i, v := range ints {
		if i != 0 && v%i == 0 {
			result = append(result, v)
		}
	}
	return
}
