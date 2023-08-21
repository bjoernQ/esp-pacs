#[doc = "Register `CORE1_DBUS_REJECT_ST` reader"]
pub type R = crate::R<CORE1_DBUS_REJECT_ST_SPEC>;
#[doc = "Field `CORE1_DBUS_TAG_ATTR` reader - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub type CORE1_DBUS_TAG_ATTR_R = crate::FieldReader;
#[doc = "Field `CORE1_DBUS_ATTR` reader - The bits are used to indicate the attribute of CPU access dbus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub type CORE1_DBUS_ATTR_R = crate::FieldReader;
#[doc = "Field `CORE1_DBUS_WORLD` reader - The bit is used to indicate the world of CPU access dbus when authentication fail. 0: WORLD0, 1: WORLD1"]
pub type CORE1_DBUS_WORLD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn core1_dbus_tag_attr(&self) -> CORE1_DBUS_TAG_ATTR_R {
        CORE1_DBUS_TAG_ATTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The bits are used to indicate the attribute of CPU access dbus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn core1_dbus_attr(&self) -> CORE1_DBUS_ATTR_R {
        CORE1_DBUS_ATTR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - The bit is used to indicate the world of CPU access dbus when authentication fail. 0: WORLD0, 1: WORLD1"]
    #[inline(always)]
    pub fn core1_dbus_world(&self) -> CORE1_DBUS_WORLD_R {
        CORE1_DBUS_WORLD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE1_DBUS_REJECT_ST")
            .field(
                "core1_dbus_tag_attr",
                &format_args!("{}", self.core1_dbus_tag_attr().bits()),
            )
            .field(
                "core1_dbus_attr",
                &format_args!("{}", self.core1_dbus_attr().bits()),
            )
            .field(
                "core1_dbus_world",
                &format_args!("{}", self.core1_dbus_world().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE1_DBUS_REJECT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_dbus_reject_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE1_DBUS_REJECT_ST_SPEC;
impl crate::RegisterSpec for CORE1_DBUS_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_dbus_reject_st::R`](R) reader structure"]
impl crate::Readable for CORE1_DBUS_REJECT_ST_SPEC {}
#[doc = "`reset()` method sets CORE1_DBUS_REJECT_ST to value 0"]
impl crate::Resettable for CORE1_DBUS_REJECT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
