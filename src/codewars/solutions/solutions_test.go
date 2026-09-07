package kata_test

import (
	. "codebook/src/codewars/solutions"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
)

var _ = Describe("What Is Between", func() {
	It("passes the sample tests", func() {
		Expect(Between(1, 4)).To(Equal([]int{1, 2, 3, 4}))
		Expect(Between(-2, 2)).To(Equal([]int{-2, -1, 0, 1, 2}))
	})
})

var _ = Describe("Take the Derivative", func() {
	It("passes the sample tests", func() {
		Expect(Derive(7, 8)).To(Equal("56x^7"))
		Expect(Derive(5, 9)).To(Equal("45x^8"))
	})
})

var _ = Describe("Basic tests", func() {
	It("should return the correct values", func() {
		Expect(MultipleOfIndex([]int{22, -6, 32, 82, 9, 25})).To(ConsistOf(-6, 32, 25))
		Expect(MultipleOfIndex([]int{68, -1, 1, -7, 10, 10})).To(ConsistOf(-1, 10))
		Expect(MultipleOfIndex([]int{11, -11})).To(ConsistOf(-11))
		Expect(MultipleOfIndex([]int{-56, -85, 72, -26, -14, 76, -27, 72, 35, -21, -67, 87, 0, 21, 59, 27, -92, 68})).To(ConsistOf(-85, 72, 0, 68))
		Expect(MultipleOfIndex([]int{28, 38, -44, -99, -13, -54, 77, -51})).To(ConsistOf(38, -44, -99))
		Expect(MultipleOfIndex([]int{-1, -49, -1, 67, 8, -60, 39, 35})).To(ConsistOf(-49, 8, -60, 35))
	})
})

var _ = Describe("Sample tests", func() {
	It("one year", func() {
		Expect(CalculateYears(1)).To(Equal([3]int{1, 15, 15}))
	})

	It("two years", func() {
		Expect(CalculateYears(2)).To(Equal([3]int{2, 24, 24}))
	})

	It("ten years", func() {
		Expect(CalculateYears(10)).To(Equal([3]int{10, 56, 64}))
	})
})

var _ = Describe("Sample Tests", func() {
	It("A square", func() {
		Expect(AreaOrPerimeter(2, 2)).To(Equal(4))
	})
	It("A rectangle", func() {
		Expect(AreaOrPerimeter(6, 10)).To(Equal(32))
	})
})

var _ = Describe("Tests", func() {
	It("Sample tests", func() {
		Expect(StringToNumber("1234")).To(Equal(1234))
		Expect(StringToNumber("605")).To(Equal(605))
		Expect(StringToNumber("1405")).To(Equal(1405))
		Expect(StringToNumber("-7")).To(Equal(-7))
	})
})
