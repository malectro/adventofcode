package main

import "fmt"
import "bufio"
import "os"
import "strings"
import "strconv"
import "regexp"

var unit_re *regexp.Regexp = regexp.MustCompile(`^(\d+)([^\d]*)$`)
var color_re *regexp.Regexp = regexp.MustCompile(`^#[0-9a-f]{6}$`)
var valid_eye_colors map[string]bool = map[string]bool{
	"amb": true, "blu": true, "brn": true, "gry": true, "grn": true, "hzl": true, "oth": true,
}

func main() {
	valid_fields := []string{
	  "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
	}

	current_passport := map[string]string{}
	total_valid_1 := 0
	total_valid_2 := 0

	s := bufio.NewScanner(os.Stdin)
	has_more := s.Scan()
    for has_more {
		line := s.Text()

		for _, item := range strings.Split(line, " ") {
			if item != "" {
				pair := strings.Split(item, ":")
				current_passport[pair[0]] = pair[1]
			}
		}

		has_more = s.Scan()

		if line == "" || !has_more {
			if hasAllFields(&valid_fields, &current_passport) {
				total_valid_1 += 1
				if isValidPassport(&current_passport) {
					total_valid_2 += 1
				}
			}
			current_passport = map[string]string{}
		}
	}

	fmt.Println(total_valid_1, total_valid_2)
}

func hasAllFields(valid_fields *[]string, passport *map[string]string) bool {
	is_valid := true
	for _, field := range *valid_fields {
		_, has_field := (*passport)[field]
		is_valid = is_valid && has_field
	}
	return is_valid
}

func isValidPassport(passport *map[string]string) bool {
	is_valid := true

	for field, value := range *passport {
		switch field {
			case "byr":
				number, _ := strconv.Atoi(value)
				is_valid = is_valid && number >= 1920 && number <= 2002
			case "iyr":
				number, _ := strconv.Atoi(value)
				is_valid = is_valid && number >= 2010 && number <= 2020
			case "eyr":
				number, _ := strconv.Atoi(value)
				is_valid = is_valid && number >= 2020 && number <= 2030
			case "hgt":
				number, unit := parseUnitValue(&value)
				if unit == "cm" {
					is_valid = is_valid && number >= 150 && number <= 193
				} else if unit == "in" {
					is_valid = is_valid && number >= 59 && number <= 76
				} else {
					is_valid = false
				}

			case "hcl":
				is_valid = is_valid && color_re.MatchString(value)

			case "ecl":
				_, is_valid_color := valid_eye_colors[value]
				is_valid = is_valid && is_valid_color

			case "pid":
				_, err := strconv.Atoi(value)
				is_valid = is_valid && len(value) == 9 && err == nil
		}
	}

	return is_valid
}

func parseUnitValue(value *string) (int, string) {
  matches := unit_re.FindStringSubmatch(*value)
  number, _ := strconv.Atoi(matches[1])
  return number, matches[2]
}
