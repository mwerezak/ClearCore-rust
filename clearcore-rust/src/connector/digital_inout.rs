




// statically track connector mode using a family of structs
struct DigitalInOut {
    
}

// possible states
struct DigitalInOut_Input {
    inout: DigitalInOut,
}

struct DigitalInOut_Output {
    inout: DigitalInOut,
}