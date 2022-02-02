package main

import "os"
import "fmt"
import "bufio"
import "strconv"

func main() {
	var numbers []uint64

	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		number, _ := strconv.ParseUint(s.Text(), 10, 64)
		numbers = append(numbers, number)
	}

	var pair []uint64
	for next, hasMore := permutations(numbers, 3); hasMore; pair, hasMore = next() {
		if sum(pair) == 2020 {
			fmt.Println(product(pair))
		}
	}
}

func permutations(slice []uint64, size uint) (func() ([]uint64, bool), bool) {
	indices := make([]int, size)
	permutation := make([]uint64, size)
	l := len(slice)

	for i := range indices {
		indices[i] = i
	}

	return func() ([]uint64, bool) {
		index := int(size) - 1

		indices[index] += 1

		for indices[index] > l-(int(size)-1-index)-1 && index > 0 {
			indices[index-1] += 1
			for i := index; i < int(size); i += 1 {
				indices[i] = indices[i-1] + 1
			}
			index -= 1
		}

		hasMore := indices[0] < l-int(size)

		for i, index := range indices {
			permutation[i] = slice[index]
		}

		return permutation, hasMore
	}, l >= int(size)
}

func sum(slice []uint64) uint64 {
	var result uint64 = 0
	for _, number := range slice {
		result += number
	}
	return result
}

func product(slice []uint64) uint64 {
	var result uint64 = 1
	for _, number := range slice {
		result *= number
	}
	return result
}
