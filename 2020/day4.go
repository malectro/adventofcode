package main

import "fmt"
import "bufio"
import "os"
import "strings"

type Path struct {
	x int;
	y int;
}

func main() {
	valid_fields := []string{
	  "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
	}
	//all_fields := append(valid_fields, "cid")

	current_passport := map[string]bool{}
	total_valid := 0

	s := bufio.NewScanner(os.Stdin)
	has_more := s.Scan()
    for has_more {
		line := s.Text()

		for _, item := range strings.Split(line, " ") {
			pair := strings.Split(item, ":")
			current_passport[pair[0]] = true
		}

		has_more = s.Scan()

		if line == "" || !has_more {
			if isValidPassport(&valid_fields, &current_passport) {
				total_valid += 1
			}
			current_passport = map[string]bool{}
		}
	}

	fmt.Println(total_valid)
}

func isValidPassport(valid_fields *[]string, passport *map[string]bool) bool {
	is_valid := true
	for _, field := range *valid_fields {
		is_valid = is_valid && (*passport)[field]
	}
	return is_valid
}
