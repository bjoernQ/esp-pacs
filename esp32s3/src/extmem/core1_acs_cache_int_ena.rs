#[doc = "Register `CORE1_ACS_CACHE_INT_ENA` reader"]
pub type R = crate::R<CORE1_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "Register `CORE1_ACS_CACHE_INT_ENA` writer"]
pub type W = crate::W<CORE1_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "Field `CORE1_IBUS_ACS_MSK_IC_INT_ENA` reader - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type CORE1_IBUS_ACS_MSK_IC_INT_ENA_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_ACS_MSK_IC_INT_ENA` writer - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
pub type CORE1_IBUS_ACS_MSK_IC_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE1_IBUS_WR_IC_INT_ENA` reader - The bit is used to enable interrupt by ibus trying to write icache"]
pub type CORE1_IBUS_WR_IC_INT_ENA_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_WR_IC_INT_ENA` writer - The bit is used to enable interrupt by ibus trying to write icache"]
pub type CORE1_IBUS_WR_IC_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE1_IBUS_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type CORE1_IBUS_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type CORE1_IBUS_REJECT_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE1_DBUS_ACS_MSK_DC_INT_ENA` reader - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type CORE1_DBUS_ACS_MSK_DC_INT_ENA_R = crate::BitReader;
#[doc = "Field `CORE1_DBUS_ACS_MSK_DC_INT_ENA` writer - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
pub type CORE1_DBUS_ACS_MSK_DC_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE1_DBUS_REJECT_INT_ENA` reader - The bit is used to enable interrupt by authentication fail."]
pub type CORE1_DBUS_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CORE1_DBUS_REJECT_INT_ENA` writer - The bit is used to enable interrupt by authentication fail."]
pub type CORE1_DBUS_REJECT_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    pub fn core1_ibus_acs_msk_ic_int_ena(&self) -> CORE1_IBUS_ACS_MSK_IC_INT_ENA_R {
        CORE1_IBUS_ACS_MSK_IC_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by ibus trying to write icache"]
    #[inline(always)]
    pub fn core1_ibus_wr_ic_int_ena(&self) -> CORE1_IBUS_WR_IC_INT_ENA_R {
        CORE1_IBUS_WR_IC_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn core1_ibus_reject_int_ena(&self) -> CORE1_IBUS_REJECT_INT_ENA_R {
        CORE1_IBUS_REJECT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    pub fn core1_dbus_acs_msk_dc_int_ena(&self) -> CORE1_DBUS_ACS_MSK_DC_INT_ENA_R {
        CORE1_DBUS_ACS_MSK_DC_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    pub fn core1_dbus_reject_int_ena(&self) -> CORE1_DBUS_REJECT_INT_ENA_R {
        CORE1_DBUS_REJECT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE1_ACS_CACHE_INT_ENA")
            .field(
                "core1_ibus_acs_msk_ic_int_ena",
                &format_args!("{}", self.core1_ibus_acs_msk_ic_int_ena().bit()),
            )
            .field(
                "core1_ibus_wr_ic_int_ena",
                &format_args!("{}", self.core1_ibus_wr_ic_int_ena().bit()),
            )
            .field(
                "core1_ibus_reject_int_ena",
                &format_args!("{}", self.core1_ibus_reject_int_ena().bit()),
            )
            .field(
                "core1_dbus_acs_msk_dc_int_ena",
                &format_args!("{}", self.core1_dbus_acs_msk_dc_int_ena().bit()),
            )
            .field(
                "core1_dbus_reject_int_ena",
                &format_args!("{}", self.core1_dbus_reject_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE1_ACS_CACHE_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_acs_msk_ic_int_ena(
        &mut self,
    ) -> CORE1_IBUS_ACS_MSK_IC_INT_ENA_W<CORE1_ACS_CACHE_INT_ENA_SPEC, 0> {
        CORE1_IBUS_ACS_MSK_IC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt by ibus trying to write icache"]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_wr_ic_int_ena(
        &mut self,
    ) -> CORE1_IBUS_WR_IC_INT_ENA_W<CORE1_ACS_CACHE_INT_ENA_SPEC, 1> {
        CORE1_IBUS_WR_IC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_reject_int_ena(
        &mut self,
    ) -> CORE1_IBUS_REJECT_INT_ENA_W<CORE1_ACS_CACHE_INT_ENA_SPEC, 2> {
        CORE1_IBUS_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to enable interrupt by cpu access dcache while the corresponding dbus is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_acs_msk_dc_int_ena(
        &mut self,
    ) -> CORE1_DBUS_ACS_MSK_DC_INT_ENA_W<CORE1_ACS_CACHE_INT_ENA_SPEC, 3> {
        CORE1_DBUS_ACS_MSK_DC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_reject_int_ena(
        &mut self,
    ) -> CORE1_DBUS_REJECT_INT_ENA_W<CORE1_ACS_CACHE_INT_ENA_SPEC, 4> {
        CORE1_DBUS_REJECT_INT_ENA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core1_acs_cache_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core1_acs_cache_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE1_ACS_CACHE_INT_ENA_SPEC;
impl crate::RegisterSpec for CORE1_ACS_CACHE_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_acs_cache_int_ena::R`](R) reader structure"]
impl crate::Readable for CORE1_ACS_CACHE_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core1_acs_cache_int_ena::W`](W) writer structure"]
impl crate::Writable for CORE1_ACS_CACHE_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE1_ACS_CACHE_INT_ENA to value 0"]
impl crate::Resettable for CORE1_ACS_CACHE_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
