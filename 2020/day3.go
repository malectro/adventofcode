package main

import "fmt"
import "bufio"
import "os"

func main() {
	total := 0
	x := 0

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		row := s.Text()
		if row[x] == '#' {
			total += 1
		}
		x = (x + 3) % len(row)
	}

	fmt.Println(total)
}

