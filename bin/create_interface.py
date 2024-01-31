import re

input_filename = "pkg/neatcoder.d.ts"
output_filename = "neatcoderInterface.d.ts"

def process_file(input_filename, output_filename):
    with open(input_filename, "r") as file:
        lines = file.readlines()

    output_lines = []
    stack = []

    # Flag to indicate if we are processing inside a class
    inside_class = False

    for line in lines:
        stripped_line = line.strip()

        # Check for class definition
        if stripped_line.startswith("export class"):
            inside_class = True
            interface_name = re.search(r'export class (\w+) {', stripped_line).group(1)
            output_lines.append(f"export interface {interface_name} {{")
            stack.append("{")
        # If inside class, process fields
        elif inside_class:
            # Check for methods by looking for lines with parentheses and ignore them
            if "(" in stripped_line or ")" in stripped_line:
                continue
            elif stripped_line.endswith(";"): ## Can either be a method or readonly field
                # If it's a readonly field, remove the readonly keyword
                # Ensure indentation when adding to output
                output_lines.append("    " + stripped_line.replace("readonly ", ""))
            elif stripped_line == "}":
                inside_class = False
                if stack:
                    stack.pop()
                    output_lines.append("}")
                    output_lines.append("")  # Add an extra line after the closing brace
        # Else, we ignore the line

    # Write the processed lines to the output file
    with open(output_filename, "w") as file:
        for line in output_lines:
            file.write(line + "\n")

process_file(input_filename, output_filename)
