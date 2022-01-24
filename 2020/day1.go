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

	var pair [2]uint64
	for next, hasMore := permutations(numbers); hasMore; pair, hasMore = next() {
		if pair[0] + pair[1] == 2020 {
			fmt.Println(pair[0], pair[1], pair[0] * pair[1])
		}
	}
}

func permutations(slice []uint64) (func () ([2]uint64, bool), bool) {
	i, j := 0, 0
	l := len(slice)

	return func () ([2]uint64, bool) {
		j += 1
		
		if j == l {
		  i += 1
		  j = i + 1
		}

		hasMore := i < l - 2
		a, b := slice[i], slice[j]

		return [2]uint64{a, b}, hasMore
	}, l > 1
}
