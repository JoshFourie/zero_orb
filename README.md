# zero_orb
Orb construction and managing for the OMNIA Protocol.

Zero Orb is a lib built in rust to streamline the orb construction and error handling in the Amelia distribution.

# HOW TO USE:

The 'Knowledge' struct holds all of the data that will be passed onto the proof scheme:

pub struct Knowledge<K, P> {

    pub wb: Vec<K>,

    pub vb: Vec<K>,

    pub wn: Vec<K>,

    pub vn: Vec<K>,

    pub ut: &'static str,

    pub pth: PathFinder<P>,
    
}

The TypeParametres <K, P> correlate with a PrimInt (u8 -> u64) and a Path. The wb, vb, wn and vn fields are for storing either witness bits, witness nums, variable bits and variable nums. Some operations, such as comparison, require that the numbers are parsed as bits and these must be placed in the wb or vb fields which are fed through the fn collect_bits() method. General operations such as + and * are able to be done as a usize and are passed through either the wn or vn fields where they are parsed with the fn collect_nums() method. The numbers MUST be fed through the correct field, and in the order they appear in the relevant .zk program. The remaining field, ut and pth, are responsible for holding the 'tag' used in fn collect_bits() to determine the number of bits to derive, and pth holds the Pathfinder struct.

pub struct PathFinder<P> {

    pub code: P,

    pub qap: P,

    pub sg1: P,

    pub sg2: P,

}


The fields in the PathFinder struct are for storing the Path references to the relevant Code, QAP, SG1 and SG2 fields. 

A proof can be generated from the values placed in the Knowledge struct by calling the .new() method, without any arguments. HOWEVER, the method does require that we provide the TypeParameters selecting the field: for instance, k.new::<FrLocal, G1Local, G2Local>().

A proof can be checked with the Marker. 

TODO: continue doc.