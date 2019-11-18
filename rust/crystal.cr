@[Link(ldflags: "#{__DIR__}/build/librust.so")]
lib LibRust
  fun number_two() : Int32
  fun str_string_hello() : UInt8*
  fun const_u8_ptr_string_hello() : UInt8*
end

puts LibRust.number_two()
puts String.new(LibRust.str_string_hello())
puts String.new(LibRust.const_u8_ptr_string_hello())
