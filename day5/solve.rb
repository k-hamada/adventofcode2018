input = open("./input.txt") {|f| f.read.chomp }
#input = "dabAcCaCBAcCcaDA"

re = Regexp.new(
  (?a..?z).flat_map {|c| [c, c.upcase] }
    .each_slice(2)
    .flat_map {|lower, upper| [lower+upper, upper+lower] }
    .join("|")
)

p (?a..?z).map {|c|
  str = input.gsub(/#{c}/i, "")

  loop do
    new_str = str.gsub(re, "")
    if str == new_str
      break
    end
    str = new_str
  end

  str.size
}.min
