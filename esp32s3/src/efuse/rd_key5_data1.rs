#[doc = "Register `RD_KEY5_DATA1` reader"]
pub type R = crate::R<RD_KEY5_DATA1_SPEC>;
#[doc = "Field `KEY5_DATA1` reader - Stores the first 32 bits of KEY5."]
pub type KEY5_DATA1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the first 32 bits of KEY5."]
    #[inline(always)]
    pub fn key5_data1(&self) -> KEY5_DATA1_R {
        KEY5_DATA1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY5_DATA1")
            .field("key5_data1", &format_args!("{}", self.key5_data1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY5_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 1 of BLOCK9 (KEY5).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY5_DATA1_SPEC;
impl crate::RegisterSpec for RD_KEY5_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key5_data1::R`](R) reader structure"]
impl crate::Readable for RD_KEY5_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_KEY5_DATA1 to value 0"]
impl crate::Resettable for RD_KEY5_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
