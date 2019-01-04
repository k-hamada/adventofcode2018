input = open("./input.txt") {|f| f.read.chomp }

re = Regexp.new(
  (?a..?z).flat_map {|c| [c, c.upcase] }
    .each_slice(2)
    .flat_map {|lower, upper| [lower+upper, upper+lower] }
    .join("|")
)

loop do
  new_str = input.gsub(re, "")
  if input == new_str
    break
  end
  input = new_str
end

p input.size
