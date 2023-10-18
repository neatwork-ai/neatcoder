import re

input_filename = "pkg/neatcoder.d.ts"
output_filename = "neatcoderInterface.d.ts"

def process_file(input_filename, output_filename):
    with open(input_filename, "r") as file:
        lines = file.readlines()

    # Store the converted lines
    output_lines = []

    # Stack to track open and close braces
    stack = []

    # Process each line
    for line in lines:
        stripped_line = line.strip()

        # Check for class definition
        if stripped_line.startswith("export class"):
            interface_name = re.search(r'export class (\w+) {', stripped_line).group(1)
            output_lines.append(f"export interface {interface_name} {{")
            stack.append("{")
        # Check for readonly fields
        elif "readonly " in stripped_line:
            output_lines.append(stripped_line.replace("readonly ", ""))
        # Check for closing braces
        elif stripped_line == "}":
            if stack:
                stack.pop()
                output_lines.append("}")
        # Else, we ignore the line

    # Write the processed lines to the output file
    with open(output_filename, "w") as file:
        for line in output_lines:
            file.write(line + "\n")

process_file(input_filename, output_filename)
