#[doc = "Register `IN_INT_RAW_CH%s` reader"]
pub struct R(crate::R<IN_INT_RAW_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_RAW_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_RAW_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_RAW_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_INT_RAW_CH%s` writer"]
pub struct W(crate::W<IN_INT_RAW_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_INT_RAW_CH_SPEC>;
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
impl From<crate::W<IN_INT_RAW_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_INT_RAW_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
pub type IN_ERR_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF` writer - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
pub type IN_ERR_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY` writer - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `INFIFO_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
#[doc = "Field `INFIFO_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_INT_RAW_CH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_ovf(&self) -> INFIFO_OVF_R {
        INFIFO_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_udf(&self) -> INFIFO_UDF_R {
        INFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<0> {
        IN_DONE_W::new(self)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<1> {
        IN_SUC_EOF_W::new(self)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<2> {
        IN_ERR_EOF_W::new(self)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<3> {
        IN_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<4> {
        IN_DSCR_EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<5> {
        INFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<6> {
        INFIFO_UDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw status interrupt of channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_raw_ch](index.html) module"]
pub struct IN_INT_RAW_CH_SPEC;
impl crate::RegisterSpec for IN_INT_RAW_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int_raw_ch::R](R) reader structure"]
impl crate::Readable for IN_INT_RAW_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_int_raw_ch::W](W) writer structure"]
impl crate::Writable for IN_INT_RAW_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_INT_RAW_CH%s to value 0"]
impl crate::Resettable for IN_INT_RAW_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}