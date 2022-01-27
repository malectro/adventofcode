package main

import "fmt"
import "bufio"
import "os"

func main() {
	maxId := 0

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		seatCode := s.Text()

		i := len(seatCode) - 3

		bounds := [...]int{0, 128}
		rowCode := seatCode[0:i]
		for _, c := range rowCode {
			midPoint := bounds[0] + (bounds[1] - bounds[0]) / 2
			switch c {
			case 'F':
				bounds[1] = midPoint
			case 'B':
				bounds[0] = midPoint
			}
		}
		row := bounds[0]

		bounds = [...]int{0, 8}
		for _, c := range seatCode[i:] {
			midPoint := bounds[0] + (bounds[1] - bounds[0]) / 2
			switch c {
			case 'L':
				bounds[1] = midPoint
			case 'R':
				bounds[0] = midPoint
			}
		}
		col := bounds[0]

		id := row * 8 + col
		if id > maxId {
			maxId = id
		}
	}

	fmt.Println(maxId)
}
