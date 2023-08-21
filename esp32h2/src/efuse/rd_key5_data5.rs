#[doc = "Register `RD_KEY5_DATA5` reader"]
pub type R = crate::R<RD_KEY5_DATA5_SPEC>;
#[doc = "Field `KEY5_DATA5` reader - Stores the fifth 32 bits of KEY5."]
pub type KEY5_DATA5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the fifth 32 bits of KEY5."]
    #[inline(always)]
    pub fn key5_data5(&self) -> KEY5_DATA5_R {
        KEY5_DATA5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY5_DATA5")
            .field("key5_data5", &format_args!("{}", self.key5_data5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY5_DATA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register $n of BLOCK9 (KEY5).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY5_DATA5_SPEC;
impl crate::RegisterSpec for RD_KEY5_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key5_data5::R`](R) reader structure"]
impl crate::Readable for RD_KEY5_DATA5_SPEC {}
#[doc = "`reset()` method sets RD_KEY5_DATA5 to value 0"]
impl crate::Resettable for RD_KEY5_DATA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
