#![cfg_attr(not(feature = "std"), no_std, no_main)]


    //--------------------------------------------------------------------------------//















    //----------------------------------------------------------------------------------//




#[ink::contract]
mod anta {


    
    #[ink(storage)]
    pub struct Anta {
        
        value: bool,
    }

    impl Anta {
        
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }


        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

    }

   
}
