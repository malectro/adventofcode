package main

import "os"
import "fmt"
import "bufio"
import "regexp"
import "strconv"

func main() {
	re := regexp.MustCompile(`^(\d+)-(\d+) ([^:]+): (.+)$`)

	totalValid := 0
	totalValid2 := 0

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		matches := re.FindStringSubmatch(s.Text())
		fmt.Println(matches)

		min, _ := strconv.Atoi(matches[1])
		max, _ := strconv.Atoi(matches[2])

		c := matches[3][0]
        password := matches[4]

		count := 0
		for i := 0; i < len(password); i++ {
			if password[i] == c {
				count += 1
			}
		}

		if count >= min && count <= max {
			totalValid += 1
		}

		a, b := password[min - 1], password[max - 1]
		if (c == a || c == b) && a != b {
			totalValid2 += 1
		}
    }

	fmt.Println("total for part 1", totalValid)
	fmt.Println("total for part 2", totalValid2)
}
