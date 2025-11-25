#!/bin/bash

# Check if day number is provided
if [ $# -eq 0 ]; then
	echo "Usage: $0 <day_number>"
	echo "Example: $0 19"
	exit 1
fi

DAY=$1

# Validate day number (should be between 1 and 25)
if ! [[ "$DAY" =~ ^[0-9]+$ ]] || [ "$DAY" -lt 1 ] || [ "$DAY" -gt 25 ]; then
	echo "Error: Day number must be between 1 and 25"
	exit 1
fi

echo "Preparing day $DAY..."

# 1. Download input file
echo "Downloading input file for day $DAY..."
INPUT_URL="https://adventofcode.com/2024/day/$DAY/input"
INPUT_FILE="d$DAY.txt"

if curl -f "$INPUT_URL" -o "$INPUT_FILE"; then
	echo "✓ Downloaded $INPUT_FILE"
else
	echo "✗ Failed to download input file from $INPUT_URL"
	echo "  You may need to add authentication or download manually"
fi

# 2. Copy template and replace day number
echo "Creating source file from template..."
SRC_FILE="src/d$DAY.rs"

if [ -f "src/d_template.rs" ]; then
	cp "src/d_template.rs" "$SRC_FILE"

	# Replace the day_no function
	sed -i "10s/todo!()/return $DAY;/" "$SRC_FILE"

	echo "✓ Created $SRC_FILE with day number $DAY"
else
	echo "✗ Template file src/d_template.rs not found"
	exit 1
fi

# 3. Update main.rs to uncomment the module import and execution
echo "Updating main.rs..."

# Uncomment module import
sed -i "s|// mod d$DAY;|mod d$DAY;|" src/main.rs

# Uncomment execution in main function
sed -i "s|// d$DAY::Task\.main();|d$DAY::Task.main();|" src/main.rs

echo "✓ Updated main.rs to include day $DAY"

echo "✅ Day $DAY preparation complete!"
echo ""
echo "Files created/modified:"
echo "  - $INPUT_FILE (input data)"
echo "  - $SRC_FILE (source code)"
echo "  - src/main.rs (updated imports and execution)"
echo ""
echo "Next steps:"
echo "  1. Edit $SRC_FILE to implement the solution"
echo "  2. Run with: cargo run"
