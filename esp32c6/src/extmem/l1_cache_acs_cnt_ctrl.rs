#[doc = "Register `L1_CACHE_ACS_CNT_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_ACS_CNT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_ACS_CNT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_ACS_CNT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_ACS_CNT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_ACS_CNT_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_ACS_CNT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_ACS_CNT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<L1_CACHE_ACS_CNT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_ACS_CNT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_IBUS0_CNT_ENA` reader - The bit is used to enable ibus0 counter in L1-ICache0."]
pub type L1_IBUS0_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS1_CNT_ENA` reader - The bit is used to enable ibus1 counter in L1-ICache1."]
pub type L1_IBUS1_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS2_CNT_ENA` reader - Reserved"]
pub type L1_IBUS2_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS3_CNT_ENA` reader - Reserved"]
pub type L1_IBUS3_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS0_CNT_ENA` reader - The bit is used to enable dbus0 counter in L1-DCache."]
pub type L1_BUS0_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS0_CNT_ENA` writer - The bit is used to enable dbus0 counter in L1-DCache."]
pub type L1_BUS0_CNT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_ACS_CNT_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_BUS1_CNT_ENA` reader - The bit is used to enable dbus1 counter in L1-DCache."]
pub type L1_BUS1_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS1_CNT_ENA` writer - The bit is used to enable dbus1 counter in L1-DCache."]
pub type L1_BUS1_CNT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_ACS_CNT_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_DBUS2_CNT_ENA` reader - Reserved"]
pub type L1_DBUS2_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_DBUS3_CNT_ENA` reader - Reserved"]
pub type L1_DBUS3_CNT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS0_CNT_CLR` reader - The bit is used to clear ibus0 counter in L1-ICache0."]
pub type L1_IBUS0_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS1_CNT_CLR` reader - The bit is used to clear ibus1 counter in L1-ICache1."]
pub type L1_IBUS1_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS2_CNT_CLR` reader - Reserved"]
pub type L1_IBUS2_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L1_IBUS3_CNT_CLR` reader - Reserved"]
pub type L1_IBUS3_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L1_BUS0_CNT_CLR` writer - The bit is used to clear dbus0 counter in L1-DCache."]
pub type L1_BUS0_CNT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_ACS_CNT_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_BUS1_CNT_CLR` writer - The bit is used to clear dbus1 counter in L1-DCache."]
pub type L1_BUS1_CNT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_ACS_CNT_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_DBUS2_CNT_CLR` reader - Reserved"]
pub type L1_DBUS2_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L1_DBUS3_CNT_CLR` reader - Reserved"]
pub type L1_DBUS3_CNT_CLR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_cnt_ena(&self) -> L1_IBUS0_CNT_ENA_R {
        L1_IBUS0_CNT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_cnt_ena(&self) -> L1_IBUS1_CNT_ENA_R {
        L1_IBUS1_CNT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_cnt_ena(&self) -> L1_IBUS2_CNT_ENA_R {
        L1_IBUS2_CNT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_cnt_ena(&self) -> L1_IBUS3_CNT_ENA_R {
        L1_IBUS3_CNT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_bus0_cnt_ena(&self) -> L1_BUS0_CNT_ENA_R {
        L1_BUS0_CNT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn l1_bus1_cnt_ena(&self) -> L1_BUS1_CNT_ENA_R {
        L1_BUS1_CNT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_cnt_ena(&self) -> L1_DBUS2_CNT_ENA_R {
        L1_DBUS2_CNT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_cnt_ena(&self) -> L1_DBUS3_CNT_ENA_R {
        L1_DBUS3_CNT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to clear ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_cnt_clr(&self) -> L1_IBUS0_CNT_CLR_R {
        L1_IBUS0_CNT_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to clear ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_cnt_clr(&self) -> L1_IBUS1_CNT_CLR_R {
        L1_IBUS1_CNT_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_cnt_clr(&self) -> L1_IBUS2_CNT_CLR_R {
        L1_IBUS2_CNT_CLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_cnt_clr(&self) -> L1_IBUS3_CNT_CLR_R {
        L1_IBUS3_CNT_CLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_cnt_clr(&self) -> L1_DBUS2_CNT_CLR_R {
        L1_DBUS2_CNT_CLR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_cnt_clr(&self) -> L1_DBUS3_CNT_CLR_R {
        L1_DBUS3_CNT_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus0_cnt_ena(&mut self) -> L1_BUS0_CNT_ENA_W<4> {
        L1_BUS0_CNT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus1_cnt_ena(&mut self) -> L1_BUS1_CNT_ENA_W<5> {
        L1_BUS1_CNT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The bit is used to clear dbus0 counter in L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus0_cnt_clr(&mut self) -> L1_BUS0_CNT_CLR_W<20> {
        L1_BUS0_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - The bit is used to clear dbus1 counter in L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_bus1_cnt_clr(&mut self) -> L1_BUS1_CNT_CLR_W<21> {
        L1_BUS1_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Access Counter enable and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_acs_cnt_ctrl](index.html) module"]
pub struct L1_CACHE_ACS_CNT_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_acs_cnt_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_ACS_CNT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_acs_cnt_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_ACS_CNT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_ACS_CNT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
