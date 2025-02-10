// Enums
//      A versatile tool used to represent a type that can take on  
//      one of several possible variants

fn main() {
    // Define an enum for IP address types
    enum IpAddressKind {
        V4,
        V6,
    }

    let _four = IpAddressKind::V4;  // :: is used to access a variant in the Enum
    let _six = IpAddressKind::V6;

    // Function that takes an IpAddressKind as a parameter
    fn route(ip_kind: IpAddressKind) {
        // match ip_kind {
        //     IpAddressKind::V4 => println!("IPv4 address"),
        //     IpAddressKind::V6 => println!("IPv6 address"),
        // }
    }

    // Call the function correctly
    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    // Using Structs with Enums
    // Passing the enum as a type of the kind variant inside the IPAdress
    struct IPAddress {
        kind: IpAddressKind,
        address: String
    }

    // Creating Instance of the IPAddress

    let home = IPAddress {
        kind: IpAddressKind::V4,
        address: String::from("192.168.0.1"),
    };

    let office = IPAddress {
        kind: IpAddressKind::V6,
        address: String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
    };


    // Using Enums to store data directly
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("2001:0db8:85a3:0000:0000:"));
    let office = IpAddr::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));



    // Ennhanced Enums
    enum IPAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let house = IPAddr::V4(127,0,0,1);
    let church = IPAddr::V6(String::from("127.0.0.1"));

}
