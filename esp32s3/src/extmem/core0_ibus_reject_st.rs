#[doc = "Register `CORE0_IBUS_REJECT_ST` reader"]
pub type R = crate::R<CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "Field `CORE0_IBUS_TAG_ATTR` reader - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub type CORE0_IBUS_TAG_ATTR_R = crate::FieldReader;
#[doc = "Field `CORE0_IBUS_ATTR` reader - The bits are used to indicate the attribute of CPU access ibus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
pub type CORE0_IBUS_ATTR_R = crate::FieldReader;
#[doc = "Field `CORE0_IBUS_WORLD` reader - The bit is used to indicate the world of CPU access ibus when authentication fail. 0: WORLD0, 1: WORLD1"]
pub type CORE0_IBUS_WORLD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn core0_ibus_tag_attr(&self) -> CORE0_IBUS_TAG_ATTR_R {
        CORE0_IBUS_TAG_ATTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The bits are used to indicate the attribute of CPU access ibus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
    #[inline(always)]
    pub fn core0_ibus_attr(&self) -> CORE0_IBUS_ATTR_R {
        CORE0_IBUS_ATTR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The bit is used to indicate the world of CPU access ibus when authentication fail. 0: WORLD0, 1: WORLD1"]
    #[inline(always)]
    pub fn core0_ibus_world(&self) -> CORE0_IBUS_WORLD_R {
        CORE0_IBUS_WORLD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_IBUS_REJECT_ST")
            .field(
                "core0_ibus_tag_attr",
                &format_args!("{}", self.core0_ibus_tag_attr().bits()),
            )
            .field(
                "core0_ibus_attr",
                &format_args!("{}", self.core0_ibus_attr().bits()),
            )
            .field(
                "core0_ibus_world",
                &format_args!("{}", self.core0_ibus_world().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE0_IBUS_REJECT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_ibus_reject_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_IBUS_REJECT_ST_SPEC;
impl crate::RegisterSpec for CORE0_IBUS_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_ibus_reject_st::R`](R) reader structure"]
impl crate::Readable for CORE0_IBUS_REJECT_ST_SPEC {}
#[doc = "`reset()` method sets CORE0_IBUS_REJECT_ST to value 0"]
impl crate::Resettable for CORE0_IBUS_REJECT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
