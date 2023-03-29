


// Connector Mode Traits
trait InputDigital {}
trait OutputDigital {}

// statically track connector mode using a family of structs
struct DigitalInOut {}
struct DigitalInOut_Input {}
struct DigitalInOut_Output {}